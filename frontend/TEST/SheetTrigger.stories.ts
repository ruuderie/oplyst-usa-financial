import type { Meta, StoryObj } from '@storybook/vue3';

import SheetTrigger from '../components/ui/sheet/SheetTrigger.vue';

const meta = {
  title: 'SheetTrigger',
  component: SheetTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetTrigger>;

export default meta;
type Story = StoryObj<typeof SheetTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};