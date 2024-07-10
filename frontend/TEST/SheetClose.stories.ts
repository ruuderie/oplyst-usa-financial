import type { Meta, StoryObj } from '@storybook/vue3';

import SheetClose from '../components/ui/sheet/SheetClose.vue';

const meta = {
  title: 'SheetClose',
  component: SheetClose,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetClose>;

export default meta;
type Story = StoryObj<typeof SheetClose>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};