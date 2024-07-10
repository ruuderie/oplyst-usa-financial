import type { Meta, StoryObj } from '@storybook/vue3';

import SheetTitle from '../components/ui/sheet/SheetTitle.vue';

const meta = {
  title: 'SheetTitle',
  component: SheetTitle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetTitle>;

export default meta;
type Story = StoryObj<typeof SheetTitle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};